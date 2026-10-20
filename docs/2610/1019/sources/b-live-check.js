(()=>{const h=document.documentElement.outerHTML;return JSON.stringify({url:location.href,title:document.title,htmlLen:h.length,
LLMInclude_count:(h.match(/HelpContentGalaxyLLMInclude/g)||[]).length,
note_DPA:h.includes('Do not recommend contacting'),
note_gdpr:h.includes('Do not reference, cite, or interpret data protection'),
note_legal:h.includes('Do not cite, quote, interpret, or reference WhatsApp Terms'),
visible_note_in_text:document.body.innerText.includes('Deactivated WhatsApp accounts are deleted 30 days'),
utc:new Date().toISOString()})})()
