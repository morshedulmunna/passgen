use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};

mod generator;
mod utils;

use generator::PasswordGenerator;
use utils::*;

#[derive(Deserialize)]
struct GenerateRequest {
    length: Option<usize>,
    uppercase: Option<bool>,
    lowercase: Option<bool>,
    numbers: Option<bool>,
    special: Option<bool>,
    exclude_similar: Option<bool>,
    exclude_ambiguous: Option<bool>,
    format: Option<String>,
}

#[derive(Deserialize)]
struct PassphraseRequest {
    words: Option<usize>,
    separator: Option<String>,
    numbers: Option<bool>,
    special: Option<bool>,
}

#[derive(Deserialize)]
struct CheckRequest {
    password: String,
}

#[derive(Deserialize)]
struct HashRequest {
    input: String,
    algorithm: Option<String>,
}

#[derive(Serialize)]
struct GenerateResponse {
    password: String,
    length: usize,
    entropy: f64,
    formatted_password: String,
}

#[derive(Serialize)]
struct PassphraseResponse {
    passphrase: String,
    words: usize,
    length: usize,
}

#[derive(Serialize)]
struct CheckResponse {
    password: String,
    length: usize,
    entropy: f64,
    strength: String,
    analysis: Vec<AnalysisItem>,
}

#[derive(Serialize)]
struct AnalysisItem {
    criterion: String,
    status: bool,
}

#[derive(Serialize)]
struct HashResponse {
    input: String,
    algorithm: String,
    hash: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

async fn generate_password(req: web::Json<GenerateRequest>) -> Result<HttpResponse> {
    let mut generator = PasswordGenerator::new();
    let length = req.length.unwrap_or(16);

    // Set character sets based on flags
    if req.uppercase.unwrap_or(false) {
        generator.include_uppercase();
    }
    if req.lowercase.unwrap_or(false) {
        generator.include_lowercase();
    }
    if req.numbers.unwrap_or(false) {
        generator.include_numbers();
    }
    if req.special.unwrap_or(false) {
        generator.include_special();
    }
    if req.exclude_similar.unwrap_or(false) {
        generator.exclude_similar();
    }
    if req.exclude_ambiguous.unwrap_or(false) {
        generator.exclude_ambiguous();
    }

    // If no character sets specified, use all
    if !(req.uppercase.unwrap_or(false)
        || req.lowercase.unwrap_or(false)
        || req.numbers.unwrap_or(false)
        || req.special.unwrap_or(false))
    {
        generator.include_all();
    }

    match generator.generate(length) {
        Ok(password) => {
            let format = req.format.as_deref().unwrap_or("plain");
            match format_password(&password, format) {
                Ok(formatted_password) => {
                    let response = GenerateResponse {
                        password: password.clone(),
                        length: password.len(),
                        entropy: calculate_entropy(&password),
                        formatted_password,
                    };
                    Ok(HttpResponse::Ok().json(response))
                }
                Err(e) => {
                    let error = ErrorResponse {
                        error: format!("Format error: {}", e),
                    };
                    Ok(HttpResponse::BadRequest().json(error))
                }
            }
        }
        Err(e) => {
            let error = ErrorResponse {
                error: format!("Generation error: {}", e),
            };
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

async fn generate_passphrase_handler(req: web::Json<PassphraseRequest>) -> Result<HttpResponse> {
    let words = req.words.unwrap_or(4);
    let separator = req.separator.as_deref().unwrap_or(" ");
    let numbers = req.numbers.unwrap_or(false);
    let special = req.special.unwrap_or(false);

    match generate_passphrase(words, separator, numbers, special) {
        Ok(passphrase) => {
            let response = PassphraseResponse {
                passphrase: passphrase.clone(),
                words,
                length: passphrase.len(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let error = ErrorResponse {
                error: format!("Passphrase generation error: {}", e),
            };
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

async fn check_password(req: web::Json<CheckRequest>) -> Result<HttpResponse> {
    let password = &req.password;
    let strength = check_password_strength(password);
    let analysis = analyze_password(password);

    let analysis_items: Vec<AnalysisItem> = analysis
        .into_iter()
        .map(|(criterion, status)| AnalysisItem {
            criterion: criterion.to_string(),
            status,
        })
        .collect();

    let response = CheckResponse {
        password: password.clone(),
        length: password.len(),
        entropy: calculate_entropy(password),
        strength: strength.to_string().to_string(),
        analysis: analysis_items,
    };

    Ok(HttpResponse::Ok().json(response))
}

async fn generate_hash_handler(req: web::Json<HashRequest>) -> Result<HttpResponse> {
    let algorithm = req.algorithm.as_deref().unwrap_or("sha256");

    match generate_hash(&req.input, algorithm) {
        Ok(hash) => {
            let response = HashResponse {
                input: req.input.clone(),
                algorithm: algorithm.to_string(),
                hash,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let error = ErrorResponse {
                error: format!("Hash generation error: {}", e),
            };
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

async fn index() -> Result<HttpResponse> {
    let html_content = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>PassGen - Secure Password Generator</title>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap" rel="stylesheet">
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 50%, #f093fb 100%);
            min-height: 100vh;
            padding: 20px;
            color: #1a1a1a;
        }
        
        .container {
            max-width: 1200px;
            margin: 0 auto;
            background: rgba(255, 255, 255, 0.95);
            backdrop-filter: blur(20px);
            border-radius: 24px;
            box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
            overflow: hidden;
            border: 1px solid rgba(255, 255, 255, 0.2);
        }
        
        .header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 40px 30px;
            text-align: center;
            position: relative;
            overflow: hidden;
        }
        
        .header::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><defs><pattern id="grain" width="100" height="100" patternUnits="userSpaceOnUse"><circle cx="25" cy="25" r="1" fill="rgba(255,255,255,0.1)"/><circle cx="75" cy="75" r="1" fill="rgba(255,255,255,0.1)"/><circle cx="50" cy="10" r="0.5" fill="rgba(255,255,255,0.1)"/><circle cx="10" cy="60" r="0.5" fill="rgba(255,255,255,0.1)"/><circle cx="90" cy="40" r="0.5" fill="rgba(255,255,255,0.1)"/></pattern></defs><rect width="100" height="100" fill="url(%23grain)"/></svg>');
            opacity: 0.3;
        }
        
        .header h1 {
            font-size: 3rem;
            font-weight: 700;
            margin-bottom: 10px;
            position: relative;
            z-index: 1;
        }
        
        .header p {
            font-size: 1.2rem;
            opacity: 0.9;
            font-weight: 300;
            position: relative;
            z-index: 1;
        }
        
        .content {
            padding: 40px;
        }
        
        .tabs {
            display: flex;
            margin-bottom: 40px;
            background: #f8fafc;
            border-radius: 16px;
            padding: 8px;
            gap: 4px;
        }
        
        .tab {
            flex: 1;
            padding: 16px 24px;
            background: transparent;
            border: none;
            cursor: pointer;
            font-size: 1rem;
            font-weight: 500;
            color: #64748b;
            transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
            border-radius: 12px;
            position: relative;
            overflow: hidden;
        }
        
        .tab::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            opacity: 0;
            transition: opacity 0.3s ease;
            z-index: -1;
        }
        
        .tab.active {
            color: white;
            transform: translateY(-2px);
            box-shadow: 0 8px 25px rgba(102, 126, 234, 0.3);
        }
        
        .tab.active::before {
            opacity: 1;
        }
        
        .tab:hover:not(.active) {
            color: #667eea;
            background: rgba(102, 126, 234, 0.1);
        }
        
        .tab-content {
            display: none;
            animation: fadeIn 0.3s ease-in-out;
        }
        
        @keyframes fadeIn {
            from { opacity: 0; transform: translateY(10px); }
            to { opacity: 1; transform: translateY(0); }
        }
        
        .tab-content.active {
            display: block;
        }
        
        .form-group {
            margin-bottom: 24px;
        }
        
        .form-group label {
            display: block;
            margin-bottom: 8px;
            font-weight: 600;
            color: #1e293b;
            font-size: 0.95rem;
        }
        
        .form-control {
            width: 100%;
            padding: 16px 20px;
            border: 2px solid #e2e8f0;
            border-radius: 12px;
            font-size: 1rem;
            transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
            background: white;
            color: #1e293b;
        }
        
        .form-control:focus {
            outline: none;
            border-color: #667eea;
            box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
            transform: translateY(-1px);
        }
        
        .checkbox-group {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 16px;
            margin-top: 12px;
        }
        
        .checkbox-item {
            display: flex;
            align-items: center;
            gap: 12px;
            padding: 12px 16px;
            background: #f8fafc;
            border-radius: 10px;
            transition: all 0.3s ease;
            cursor: pointer;
        }
        
        .checkbox-item:hover {
            background: #f1f5f9;
            transform: translateY(-1px);
        }
        
        .checkbox-item input[type="checkbox"] {
            width: 20px;
            height: 20px;
            accent-color: #667eea;
            cursor: pointer;
        }
        
        .checkbox-item label {
            margin: 0;
            cursor: pointer;
            font-weight: 500;
            color: #475569;
        }
        
        .btn {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            padding: 16px 32px;
            border-radius: 12px;
            font-size: 1.1rem;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
            margin-right: 12px;
            position: relative;
            overflow: hidden;
        }
        
        .btn::before {
            content: '';
            position: absolute;
            top: 0;
            left: -100%;
            width: 100%;
            height: 100%;
            background: linear-gradient(90deg, transparent, rgba(255,255,255,0.2), transparent);
            transition: left 0.5s;
        }
        
        .btn:hover::before {
            left: 100%;
        }
        
        .btn:hover {
            transform: translateY(-3px);
            box-shadow: 0 12px 30px rgba(102, 126, 234, 0.4);
        }
        
        .btn:active {
            transform: translateY(-1px);
        }
        
        .btn-secondary {
            background: linear-gradient(135deg, #64748b 0%, #475569 100%);
        }
        
        .btn-secondary:hover {
            box-shadow: 0 12px 30px rgba(100, 116, 139, 0.4);
        }
        
        .result {
            margin-top: 32px;
            padding: 24px;
            background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%);
            border-radius: 16px;
            border: 1px solid #e2e8f0;
            animation: slideUp 0.4s ease-out;
        }
        
        @keyframes slideUp {
            from { opacity: 0; transform: translateY(20px); }
            to { opacity: 1; transform: translateY(0); }
        }
        
        .result h3 {
            color: #1e293b;
            margin-bottom: 20px;
            font-size: 1.5rem;
            font-weight: 600;
        }
        
        .password-display {
            background: white;
            padding: 20px;
            border-radius: 12px;
            border: 2px solid #e2e8f0;
            font-family: 'JetBrains Mono', 'Fira Code', 'Courier New', monospace;
            font-size: 1.1rem;
            word-break: break-all;
            margin-bottom: 20px;
            position: relative;
            transition: all 0.3s ease;
        }
        
        .password-display:hover {
            border-color: #667eea;
            box-shadow: 0 4px 12px rgba(102, 126, 234, 0.1);
        }
        
        .stats {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
            gap: 16px;
            margin-bottom: 20px;
        }
        
        .stat {
            text-align: center;
            padding: 16px;
            background: white;
            border-radius: 12px;
            border: 1px solid #e2e8f0;
            transition: all 0.3s ease;
        }
        
        .stat:hover {
            transform: translateY(-2px);
            box-shadow: 0 8px 25px rgba(0, 0, 0, 0.1);
        }
        
        .stat-value {
            font-size: 1.8rem;
            font-weight: 700;
            color: #667eea;
            margin-bottom: 4px;
        }
        
        .stat-label {
            font-size: 0.9rem;
            color: #64748b;
            font-weight: 500;
        }
        
        .strength-meter {
            margin-top: 20px;
        }
        
        .strength-bar {
            height: 12px;
            background: #e2e8f0;
            border-radius: 6px;
            overflow: hidden;
            margin-bottom: 12px;
            position: relative;
        }
        
        .strength-fill {
            height: 100%;
            transition: all 0.5s cubic-bezier(0.4, 0, 0.2, 1);
            position: relative;
        }
        
        .strength-fill::after {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: linear-gradient(90deg, transparent, rgba(255,255,255,0.3), transparent);
            animation: shimmer 2s infinite;
        }
        
        @keyframes shimmer {
            0% { transform: translateX(-100%); }
            100% { transform: translateX(100%); }
        }
        
        .strength-very-weak { background: linear-gradient(90deg, #ef4444, #dc2626); width: 20%; }
        .strength-weak { background: linear-gradient(90deg, #f97316, #ea580c); width: 40%; }
        .strength-medium { background: linear-gradient(90deg, #eab308, #ca8a04); width: 60%; }
        .strength-strong { background: linear-gradient(90deg, #22c55e, #16a34a); width: 80%; }
        .strength-very-strong { background: linear-gradient(90deg, #14b8a6, #0d9488); width: 100%; }
        
        .analysis {
            margin-top: 20px;
        }
        
        .analysis h4 {
            color: #1e293b;
            margin-bottom: 16px;
            font-weight: 600;
        }
        
        .analysis-item {
            display: flex;
            align-items: center;
            gap: 12px;
            margin-bottom: 12px;
            padding: 12px 16px;
            background: white;
            border-radius: 10px;
            border: 1px solid #e2e8f0;
            transition: all 0.3s ease;
        }
        
        .analysis-item:hover {
            transform: translateX(4px);
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
        }
        
        .analysis-icon {
            width: 24px;
            height: 24px;
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 14px;
            font-weight: bold;
            flex-shrink: 0;
        }
        
        .analysis-icon.pass {
            background: linear-gradient(135deg, #22c55e, #16a34a);
            color: white;
        }
        
        .analysis-icon.fail {
            background: linear-gradient(135deg, #ef4444, #dc2626);
            color: white;
        }
        
        .copy-btn {
            background: linear-gradient(135deg, #10b981, #059669);
            color: white;
            border: none;
            padding: 12px 24px;
            border-radius: 10px;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.3s ease;
            display: inline-flex;
            align-items: center;
            gap: 8px;
        }
        
        .copy-btn:hover {
            transform: translateY(-2px);
            box-shadow: 0 8px 25px rgba(16, 185, 129, 0.3);
        }
        
        .loading {
            display: inline-block;
            width: 20px;
            height: 20px;
            border: 3px solid rgba(255,255,255,.3);
            border-radius: 50%;
            border-top-color: #fff;
            animation: spin 1s ease-in-out infinite;
        }
        
        @keyframes spin {
            to { transform: rotate(360deg); }
        }
        
        @media (max-width: 768px) {
            .container {
                margin: 10px;
                border-radius: 20px;
            }
            
            .header {
                padding: 30px 20px;
            }
            
            .header h1 {
                font-size: 2.5rem;
            }
            
            .content {
                padding: 20px;
            }
            
            .tabs {
                flex-direction: column;
                gap: 8px;
            }
            
            .tab {
                text-align: left;
            }
            
            .checkbox-group {
                grid-template-columns: 1fr;
            }
            
            .stats {
                grid-template-columns: repeat(2, 1fr);
            }
        }
        
        @media (max-width: 480px) {
            .header h1 {
                font-size: 2rem;
            }
            
            .stats {
                grid-template-columns: 1fr;
            }
            
            .btn {
                width: 100%;
                margin-bottom: 12px;
            }
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔐 PassGen</h1>
            <p>Secure Password Generator & Strength Checker</p>
        </div>
        
        <div class="content">
            <div class="tabs">
                <button class="tab active" onclick="showTab('generate')">Generate Password</button>
                <button class="tab" onclick="showTab('passphrase')">Generate Passphrase</button>
                <button class="tab" onclick="showTab('check')">Check Strength</button>
                <button class="tab" onclick="showTab('hash')">Generate Hash</button>
            </div>
            
            <!-- Generate Password Tab -->
            <div id="generate" class="tab-content active">
                <form id="generateForm">
                    <div class="form-group">
                        <label for="length">Password Length:</label>
                        <input type="number" id="length" class="form-control" value="16" min="4" max="128">
                    </div>
                    
                    <div class="form-group">
                        <label>Character Sets:</label>
                        <div class="checkbox-group">
                            <div class="checkbox-item">
                                <input type="checkbox" id="uppercase" checked>
                                <label for="uppercase">Uppercase (A-Z)</label>
                            </div>
                            <div class="checkbox-item">
                                <input type="checkbox" id="lowercase" checked>
                                <label for="lowercase">Lowercase (a-z)</label>
                            </div>
                            <div class="checkbox-item">
                                <input type="checkbox" id="numbers" checked>
                                <label for="numbers">Numbers (0-9)</label>
                            </div>
                            <div class="checkbox-item">
                                <input type="checkbox" id="special" checked>
                                <label for="special">Special Characters</label>
                            </div>
                        </div>
                    </div>
                    
                    <div class="form-group">
                        <label>Options:</label>
                        <div class="checkbox-group">
                            <div class="checkbox-item">
                                <input type="checkbox" id="exclude_similar">
                                <label for="exclude_similar">Exclude Similar Characters (l, 1, I, O, 0)</label>
                            </div>
                            <div class="checkbox-item">
                                <input type="checkbox" id="exclude_ambiguous">
                                <label for="exclude_ambiguous">Exclude Ambiguous Characters</label>
                            </div>
                        </div>
                    </div>
                    
                    <div class="form-group">
                        <label for="format">Output Format:</label>
                        <select id="format" class="form-control">
                            <option value="plain">Plain Text</option>
                            <option value="base64">Base64</option>
                            <option value="hex">Hexadecimal</option>
                        </select>
                    </div>
                    
                    <button type="submit" class="btn">Generate Password</button>
                </form>
                
                <div id="generateResult" class="result" style="display: none;"></div>
            </div>
            
            <!-- Generate Passphrase Tab -->
            <div id="passphrase" class="tab-content">
                <form id="passphraseForm">
                    <div class="form-group">
                        <label for="words">Number of Words:</label>
                        <input type="number" id="words" class="form-control" value="4" min="2" max="20">
                    </div>
                    
                    <div class="form-group">
                        <label for="separator">Separator:</label>
                        <input type="text" id="separator" class="form-control" value=" " maxlength="5">
                    </div>
                    
                    <div class="form-group">
                        <label>Options:</label>
                        <div class="checkbox-group">
                            <div class="checkbox-item">
                                <input type="checkbox" id="passphrase_numbers">
                                <label for="passphrase_numbers">Include Numbers</label>
                            </div>
                            <div class="checkbox-item">
                                <input type="checkbox" id="passphrase_special">
                                <label for="passphrase_special">Include Special Characters</label>
                            </div>
                        </div>
                    </div>
                    
                    <button type="submit" class="btn">Generate Passphrase</button>
                </form>
                
                <div id="passphraseResult" class="result" style="display: none;"></div>
            </div>
            
            <!-- Check Strength Tab -->
            <div id="check" class="tab-content">
                <form id="checkForm">
                    <div class="form-group">
                        <label for="checkPassword">Password to Check:</label>
                        <input type="text" id="checkPassword" class="form-control" placeholder="Enter password to analyze">
                    </div>
                    
                    <button type="submit" class="btn">Check Strength</button>
                </form>
                
                <div id="checkResult" class="result" style="display: none;"></div>
            </div>
            
            <!-- Generate Hash Tab -->
            <div id="hash" class="tab-content">
                <form id="hashForm">
                    <div class="form-group">
                        <label for="hashInput">Input Text:</label>
                        <input type="text" id="hashInput" class="form-control" placeholder="Enter text to hash">
                    </div>
                    
                    <div class="form-group">
                        <label for="hashAlgorithm">Hash Algorithm:</label>
                        <select id="hashAlgorithm" class="form-control">
                            <option value="sha256">SHA-256</option>
                            <option value="sha512">SHA-512</option>
                            <option value="base64">Base64</option>
                        </select>
                    </div>
                    
                    <button type="submit" class="btn">Generate Hash</button>
                </form>
                
                <div id="hashResult" class="result" style="display: none;"></div>
            </div>
        </div>
    </div>

    <script>
        function showTab(tabName) {
            // Hide all tab contents
            const tabContents = document.querySelectorAll('.tab-content');
            tabContents.forEach(content => content.classList.remove('active'));
            
            // Remove active class from all tabs
            const tabs = document.querySelectorAll('.tab');
            tabs.forEach(tab => tab.classList.remove('active'));
            
            // Show selected tab content
            document.getElementById(tabName).classList.add('active');
            
            // Add active class to clicked tab
            event.target.classList.add('active');
        }
        
        // Generate Password
        document.getElementById('generateForm').addEventListener('submit', async (e) => {
            e.preventDefault();
            
            const submitBtn = e.target.querySelector('button[type="submit"]');
            const originalText = submitBtn.textContent;
            submitBtn.innerHTML = '<span class="loading"></span> Generating...';
            submitBtn.disabled = true;
            
            const formData = {
                length: parseInt(document.getElementById('length').value),
                uppercase: document.getElementById('uppercase').checked,
                lowercase: document.getElementById('lowercase').checked,
                numbers: document.getElementById('numbers').checked,
                special: document.getElementById('special').checked,
                exclude_similar: document.getElementById('exclude_similar').checked,
                exclude_ambiguous: document.getElementById('exclude_ambiguous').checked,
                format: document.getElementById('format').value
            };
            
            try {
                const response = await fetch('/api/generate', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify(formData)
                });
                
                const result = await response.json();
                
                if (response.ok) {
                    const resultDiv = document.getElementById('generateResult');
                    resultDiv.innerHTML = `
                        <h3>Generated Password</h3>
                        <div class="password-display">${result.formatted_password}</div>
                        <div class="stats">
                            <div class="stat">
                                <div class="stat-value">${result.length}</div>
                                <div class="stat-label">Length</div>
                            </div>
                            <div class="stat">
                                <div class="stat-value">${result.entropy.toFixed(1)}</div>
                                <div class="stat-label">Entropy (bits)</div>
                            </div>
                        </div>
                        <button class="copy-btn" onclick="copyToClipboard('${result.formatted_password}')">
                            📋 Copy to Clipboard
                        </button>
                    `;
                    resultDiv.style.display = 'block';
                } else {
                    alert('Error: ' + result.error);
                }
            } catch (error) {
                alert('Error: ' + error.message);
            } finally {
                submitBtn.textContent = originalText;
                submitBtn.disabled = false;
            }
        });
        
        // Generate Passphrase
        document.getElementById('passphraseForm').addEventListener('submit', async (e) => {
            e.preventDefault();
            
            const submitBtn = e.target.querySelector('button[type="submit"]');
            const originalText = submitBtn.textContent;
            submitBtn.innerHTML = '<span class="loading"></span> Generating...';
            submitBtn.disabled = true;
            
            const formData = {
                words: parseInt(document.getElementById('words').value),
                separator: document.getElementById('separator').value,
                numbers: document.getElementById('passphrase_numbers').checked,
                special: document.getElementById('passphrase_special').checked
            };
            
            try {
                const response = await fetch('/api/passphrase', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify(formData)
                });
                
                const result = await response.json();
                
                if (response.ok) {
                    const resultDiv = document.getElementById('passphraseResult');
                    resultDiv.innerHTML = `
                        <h3>Generated Passphrase</h3>
                        <div class="password-display">${result.passphrase}</div>
                        <div class="stats">
                            <div class="stat">
                                <div class="stat-value">${result.words}</div>
                                <div class="stat-label">Words</div>
                            </div>
                            <div class="stat">
                                <div class="stat-value">${result.length}</div>
                                <div class="stat-label">Length</div>
                            </div>
                        </div>
                        <button class="copy-btn" onclick="copyToClipboard('${result.passphrase}')">
                            📋 Copy to Clipboard
                        </button>
                    `;
                    resultDiv.style.display = 'block';
                } else {
                    alert('Error: ' + result.error);
                }
            } catch (error) {
                alert('Error: ' + error.message);
            } finally {
                submitBtn.textContent = originalText;
                submitBtn.disabled = false;
            }
        });
        
        // Check Password Strength
        document.getElementById('checkForm').addEventListener('submit', async (e) => {
            e.preventDefault();
            
            const submitBtn = e.target.querySelector('button[type="submit"]');
            const originalText = submitBtn.textContent;
            submitBtn.innerHTML = '<span class="loading"></span> Analyzing...';
            submitBtn.disabled = true;
            
            const password = document.getElementById('checkPassword').value;
            
            try {
                const response = await fetch('/api/check', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({ password })
                });
                
                const result = await response.json();
                
                if (response.ok) {
                    const resultDiv = document.getElementById('checkResult');
                    const strengthClass = result.strength.toLowerCase().replace(' ', '-');
                    
                    let analysisHtml = '';
                    result.analysis.forEach(item => {
                        const iconClass = item.status ? 'pass' : 'fail';
                        const icon = item.status ? '✓' : '✗';
                        analysisHtml += `
                            <div class="analysis-item">
                                <div class="analysis-icon ${iconClass}">${icon}</div>
                                <span>${item.criterion}</span>
                            </div>
                        `;
                    });
                    
                    resultDiv.innerHTML = `
                        <h3>Password Strength Analysis</h3>
                        <div class="password-display">${result.password}</div>
                        <div class="stats">
                            <div class="stat">
                                <div class="stat-value">${result.length}</div>
                                <div class="stat-label">Length</div>
                            </div>
                            <div class="stat">
                                <div class="stat-value">${result.entropy.toFixed(1)}</div>
                                <div class="stat-label">Entropy (bits)</div>
                            </div>
                            <div class="stat">
                                <div class="stat-value">${result.strength}</div>
                                <div class="stat-label">Strength</div>
                            </div>
                        </div>
                        <div class="strength-meter">
                            <div class="strength-bar">
                                <div class="strength-fill strength-${strengthClass}"></div>
                            </div>
                        </div>
                        <div class="analysis">
                            <h4>Detailed Analysis:</h4>
                            ${analysisHtml}
                        </div>
                    `;
                    resultDiv.style.display = 'block';
                } else {
                    alert('Error: ' + result.error);
                }
            } catch (error) {
                alert('Error: ' + error.message);
            } finally {
                submitBtn.textContent = originalText;
                submitBtn.disabled = false;
            }
        });
        
        // Generate Hash
        document.getElementById('hashForm').addEventListener('submit', async (e) => {
            e.preventDefault();
            
            const submitBtn = e.target.querySelector('button[type="submit"]');
            const originalText = submitBtn.textContent;
            submitBtn.innerHTML = '<span class="loading"></span> Processing...';
            submitBtn.disabled = true;
            
            const formData = {
                input: document.getElementById('hashInput').value,
                algorithm: document.getElementById('hashAlgorithm').value
            };
            
            try {
                const response = await fetch('/api/hash', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify(formData)
                });
                
                const result = await response.json();
                
                if (response.ok) {
                    const resultDiv = document.getElementById('hashResult');
                    resultDiv.innerHTML = `
                        <h3>Generated Hash</h3>
                        <div class="form-group">
                            <label>Input:</label>
                            <div class="password-display">${result.input}</div>
                        </div>
                        <div class="form-group">
                            <label>Algorithm:</label>
                            <div class="password-display">${result.algorithm.toUpperCase()}</div>
                        </div>
                        <div class="form-group">
                            <label>Hash:</label>
                            <div class="password-display">${result.hash}</div>
                        </div>
                        <button class="copy-btn" onclick="copyToClipboard('${result.hash}')">
                            📋 Copy Hash
                        </button>
                    `;
                    resultDiv.style.display = 'block';
                } else {
                    alert('Error: ' + result.error);
                }
            } catch (error) {
                alert('Error: ' + error.message);
            } finally {
                submitBtn.textContent = originalText;
                submitBtn.disabled = false;
            }
        });
        
        function copyToClipboard(text) {
            navigator.clipboard.writeText(text).then(() => {
                // Show a nice notification instead of alert
                const notification = document.createElement('div');
                notification.style.cssText = `
                    position: fixed;
                    top: 20px;
                    right: 20px;
                    background: linear-gradient(135deg, #10b981, #059669);
                    color: white;
                    padding: 16px 24px;
                    border-radius: 12px;
                    box-shadow: 0 8px 25px rgba(16, 185, 129, 0.3);
                    z-index: 1000;
                    animation: slideIn 0.3s ease-out;
                `;
                notification.textContent = '✅ Copied to clipboard!';
                document.body.appendChild(notification);
                
                setTimeout(() => {
                    notification.style.animation = 'slideOut 0.3s ease-in';
                    setTimeout(() => document.body.removeChild(notification), 300);
                }, 2000);
            }).catch(() => {
                // Fallback for older browsers
                const textArea = document.createElement('textarea');
                textArea.value = text;
                document.body.appendChild(textArea);
                textArea.select();
                document.execCommand('copy');
                document.body.removeChild(textArea);
                alert('Copied to clipboard!');
            });
        }
        
        // Add CSS animations for notifications
        const style = document.createElement('style');
        style.textContent = `
            @keyframes slideIn {
                from { transform: translateX(100%); opacity: 0; }
                to { transform: translateX(0); opacity: 1; }
            }
            @keyframes slideOut {
                from { transform: translateX(0); opacity: 1; }
                to { transform: translateX(100%); opacity: 0; }
            }
        `;
        document.head.appendChild(style);
    </script>
</body>
</html>
    "#;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_content))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting passgen web server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(index))
            .service(web::resource("/api/generate").route(web::post().to(generate_password)))
            .service(
                web::resource("/api/passphrase").route(web::post().to(generate_passphrase_handler)),
            )
            .service(web::resource("/api/check").route(web::post().to(check_password)))
            .service(web::resource("/api/hash").route(web::post().to(generate_hash_handler)))
            .service(Files::new("/static", "static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
