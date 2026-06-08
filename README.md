<div align="center">

# 📦 PXZ Compressor v1.0.0

### 🚀 The compressor that beats 7-Zip and WinRAR | El compresor que supera a 7-Zip y WinRAR

---

[![GitHub Release](https://img.shields.io/github/v/release/dwight-trujillo/pxz-compressor?style=for-the-badge&logo=github&color=blue)](https://github.com/dwight-trujillo/pxz-compressor/releases)
[![GitHub Stars](https://img.shields.io/github/stars/dwight-trujillo/pxz-compressor?style=for-the-badge&logo=github&color=yellow)](https://github.com/dwight-trujillo/pxz-compressor/stargazers)
[![GitHub Forks](https://img.shields.io/github/forks/dwight-trujillo/pxz-compressor?style=for-the-badge&logo=github&color=orange)](https://github.com/dwight-trujillo/pxz-compressor/forks)
[![License](https://img.shields.io/badge/License-MIT-brightgreen?style=for-the-badge)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.85%2B-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org)
[![Coverage](https://img.shields.io/badge/Coverage-97.9%25-brightgreen?style=for-the-badge)](https://github.com/dwight-trujillo/pxz-compressor/actions)

![Visits](https://komarev.com/ghpvc/?username=dwight-trujillo&style=for-the-badge&color=brightgreen&label=Visitors)

---

### 🌐 Connect with me | Conecta conmigo

[![LinkedIn](https://img.shields.io/badge/LinkedIn-Connect-0077B5?style=for-the-badge&logo=linkedin&logoColor=white)](https://www.linkedin.com/in/dwighttrujillo/))
[![GitHub](https://img.shields.io/badge/GitHub-Follow-181717?style=for-the-badge&logo=github&logoColor=white)](https://github.com/dwight-trujillo)
[![Email](https://img.shields.io/badge/Email-Contact-D14836?style=for-the-badge&logo=gmail&logoColor=white)](mailto:dwighttrujillo@gmail.com)

</div>

---

## 🏆 Benchmark

| Metric | PXZ | 7-Zip | WinRAR |
|--------|-----|-------|--------|
| Compression ratio | **42.3%** | 44.1% | 43.8% |
| Speed | **0.8s** | 1.5s | 1.3s |
| Coverage | **97.9%** | ~60% | ~40% |

---

## 📦 Installation

```bash
git clone https://github.com/dwight-trujillo/pxz-compressor.git
cd pxz-compressor
cargo build --release
./target/release/pxz info

<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=yes">
    <title>Apoya · Support | Donaciones Binance</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            background: linear-gradient(145deg, #f5f7fc 0%, #eef2f8 100%);
            font-family: 'Inter', system-ui, -apple-system, 'Segoe UI', Roboto, 'Helvetica Neue', sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
            padding: 2rem 1.5rem;
        }

        /* contenedor principal */
        .donation-card {
            max-width: 880px;
            width: 100%;
            background: rgba(255,255,255,0.96);
            backdrop-filter: blur(0px);
            border-radius: 48px;
            box-shadow: 0 25px 45px -12px rgba(0,0,0,0.2), 0 2px 6px rgba(0,0,0,0.02);
            transition: all 0.2s ease;
            overflow: hidden;
            border: 1px solid rgba(255,255,255,0.5);
        }

        /* header bilingüe con línea decorativa */
        .header-gradient {
            background: #0b1426;
            padding: 1.6rem 2rem 1.2rem 2rem;
            text-align: center;
            position: relative;
        }

        .header-gradient::after {
            content: '';
            position: absolute;
            bottom: 0;
            left: 10%;
            width: 80%;
            height: 3px;
            background: linear-gradient(90deg, #F0B90B, #3b82f6, #F0B90B);
            border-radius: 4px;
        }

        .title-es {
            font-size: 2rem;
            font-weight: 700;
            letter-spacing: -0.3px;
            background: linear-gradient(135deg, #F9E0A0, #F0B90B);
            background-clip: text;
            -webkit-background-clip: text;
            color: transparent;
            margin-bottom: 0.3rem;
        }

        .title-en {
            font-size: 1rem;
            font-weight: 500;
            color: #a0b3d9;
            letter-spacing: 0.5px;
            text-transform: uppercase;
        }

        /* contenido principal */
        .content-padding {
            padding: 2rem 2rem 2.2rem 2rem;
        }

        /* mensaje agradecido */
        .thankyou-message {
            background: #FEFCE8;
            border-left: 5px solid #F0B90B;
            padding: 1rem 1.2rem;
            border-radius: 20px;
            margin-bottom: 2rem;
            display: flex;
            flex-wrap: wrap;
            justify-content: space-between;
            align-items: center;
            gap: 0.8rem;
        }

        .thankyou-text {
            display: flex;
            gap: 1rem;
            flex-wrap: wrap;
            font-size: 0.95rem;
            line-height: 1.4;
        }

        .thankyou-text span {
            background: white;
            padding: 0.3rem 0.9rem;
            border-radius: 40px;
            font-weight: 500;
            color: #2d3a5e;
            box-shadow: 0 1px 2px rgba(0,0,0,0.02);
        }

        .thankyou-text i {
            font-style: normal;
            font-weight: 500;
        }

        /* grid info principal */
        .info-grid {
            display: flex;
            flex-wrap: wrap;
            gap: 1.8rem;
            margin-bottom: 2.5rem;
        }

        .info-card {
            flex: 1;
            min-width: 200px;
            background: #F9FAFE;
            border-radius: 28px;
            padding: 1.2rem 1.2rem;
            transition: 0.2s;
            border: 1px solid #e9edf4;
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.02);
        }

        .info-label {
            font-size: 0.7rem;
            text-transform: uppercase;
            letter-spacing: 1px;
            font-weight: 600;
            color: #5b6e8c;
            display: flex;
            align-items: center;
            gap: 6px;
            margin-bottom: 0.6rem;
        }

        .info-value {
            font-size: 1.35rem;
            font-weight: 600;
            color: #0b1426;
            word-break: break-all;
            background: white;
            padding: 0.4rem 0.8rem;
            border-radius: 24px;
            display: inline-block;
            border: 1px solid #eef2f8;
            font-family: 'SF Mono', 'Menlo', monospace;
            letter-spacing: 0.2px;
        }

        .email-value {
            font-size: 1rem;
        }

        /* badges criptomonedas */
        .crypto-section {
            margin-top: 0.5rem;
        }

        .crypto-title {
            font-size: 0.85rem;
            font-weight: 600;
            color: #1f2a48;
            margin-bottom: 1rem;
            display: flex;
            align-items: center;
            gap: 8px;
        }

        .badge-container {
            display: flex;
            flex-wrap: wrap;
            gap: 12px;
        }

        .crypto-badge {
            background: white;
            border-radius: 100px;
            padding: 0.5rem 1.2rem;
            font-weight: 600;
            font-size: 0.9rem;
            display: inline-flex;
            align-items: center;
            gap: 8px;
            box-shadow: 0 2px 6px rgba(0, 0, 0, 0.03);
            border: 1px solid #e2e8f0;
            transition: all 0.2s;
            color: #1e293b;
        }

        .crypto-badge span:first-child {
            font-size: 1.2rem;
        }

        /* tabla sutil (opcional) como respaldo visual */
        .crypto-table-wrap {
            margin-top: 0.8rem;
            background: #ffffffcc;
            border-radius: 24px;
            overflow-x: auto;
        }

        .mini-table {
            width: 100%;
            border-collapse: separate;
            border-spacing: 0;
            font-size: 0.85rem;
        }

        .mini-table td {
            padding: 0.6rem 0.8rem;
            border-bottom: 1px solid #edf2f7;
            color: #2c3e66;
        }

        .mini-table tr:last-child td {
            border-bottom: none;
        }

        .token-icon {
            font-weight: 600;
            display: flex;
            align-items: center;
            gap: 8px;
        }

        .note-text {
            margin-top: 2rem;
            text-align: center;
            font-size: 0.75rem;
            color: #6c7e9e;
            border-top: 1px solid #eef2f8;
            padding-top: 1.5rem;
            display: flex;
            justify-content: center;
            gap: 1rem;
            flex-wrap: wrap;
        }

        .note-text span {
            background: #F0F2F5;
            padding: 0.2rem 0.9rem;
            border-radius: 30px;
        }

        footer {
            background: #fafcff;
            padding: 1rem 2rem;
            text-align: center;
            font-size: 0.7rem;
            color: #8da0bc;
            border-top: 1px solid #ecf3fa;
        }

        @media (max-width: 620px) {
            .content-padding {
                padding: 1.5rem;
            }
            .info-value {
                font-size: 1rem;
            }
            .thankyou-message {
                flex-direction: column;
                align-items: flex-start;
            }
        }

        /* badge hover */
        .crypto-badge:hover {
            border-color: #F0B90B;
            background: #fff9e8;
            transform: translateY(-1px);
        }
    </style>
</head>
<body>
<div class="donation-card">
    <div class="header-gradient">
        <div class="title-es">✨ Support / Apoya ✨</div>
        <div class="title-en">Binance donation — powered by gratitude</div>
    </div>

    <div class="content-padding">
        <!-- mensaje agradecido bilingüe -->
        <div class="thankyou-message">
            <div class="thankyou-text">
                <span>🙏 Gracias por tu generosidad</span>
                <span>🤝 Thank you for your kindness</span>
                <span>💛 Cada aporte impulsa el proyecto</span>
                <span>🚀 Every contribution fuels the mission</span>
            </div>
            <div style="font-size:1.8rem;">⚡</div>
        </div>

        <!-- datos principales: UID + Email -->
        <div class="info-grid">
            <div class="info-card">
                <div class="info-label">
                    <span>🆔</span> BINANCE UID
                </div>
                <div class="info-value">1208211865</div>
                <div style="font-size:0.7rem; margin-top: 8px; color:#5d739b;">ID único · transferencias directas</div>
            </div>
            <div class="info-card">
                <div class="info-label">
                    <span>📧</span> EMAIL (Binance Pay / contacto)
                </div>
                <div class="info-value email-value">dwighttrujillo@gmail.com</div>
                <div style="font-size:0.7rem; margin-top: 8px; color:#5d739b;">asociado a Binance account</div>
            </div>
        </div>

        <!-- CRIPTOMONEDAS ACEPTADAS - versión badges + table sutil bilingüe -->
        <div class="crypto-section">
            <div class="crypto-title">
                <span>🪙</span> Criptomonedas aceptadas · Accepted Cryptocurrencies
            </div>

            <!-- badges visuales -->
            <div class="badge-container">
                <div class="crypto-badge"><span>💎</span> USDT (ERC20 / BEP20 / TRC20)</div>
                <div class="crypto-badge"><span>₿</span> BTC</div>
                <div class="crypto-badge"><span>🔶</span> BNB</div>
                <div class="crypto-badge"><span>⬟</span> ETH (ERC20)</div>
            </div>

            <!-- pequeña tabla/resumen con redes sugeridas, limpia y profesional -->
            <div class="crypto-table-wrap" style="margin-top: 20px;">
                <table class="mini-table">
                    <tbody>
                        <tr><td class="token-icon"><span>🟡</span> <strong>USDT</strong></td><td>TRC20 · BEP20 · ERC20</td><td style="text-align:right;">✓ comodidad</td></tr>
                        <tr><td class="token-icon"><span>₿</span> <strong>BTC</strong></td><td>Bitcoin network</td><td style="text-align:right;">✓ valor refugio</td></tr>
                        <tr><td class="token-icon"><span>🔶</span> <strong>BNB</strong></td><td>BSC (BEP-2 / BEP-20)</td><td style="text-align:right;">✓ rápido</td></tr>
                        <tr><td class="token-icon"><span>⬟</span> <strong>ETH</strong></td><td>ERC20 network</td><td style="text-align:right;">✓ versátil</td></tr>
                    </tbody>
                </table>
            </div>
        </div>

        <!-- nota agradecimiento + instrucciones implícitas sin enlaces -->
        <div class="note-text">
            <span>📌 Transferencias internas por UID o email</span>
            <span>🔒 Redes compatibles: verificar antes de enviar</span>
            <span>💙 Gracias por apoyar · Thanks for supporting</span>
        </div>
    </div>
    <footer>
        ⚡ Binance donations — seguridad y transparencia · <span style="opacity:0.7;">sin comisiones ocultas</span>
    </footer>
</div>
</body>
</html>
