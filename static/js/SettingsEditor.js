import"./MarkdownEditor-1a13690f1612bf18.js";function O(q,A,h,C){if(C===void 0)C="paste";const I=document.getElementById("update-form"),M=document.getElementById("add_field");let B="",E="";window.change_current_property=(D)=>{const G=D.target.options[D.target.selectedIndex];if(G){if(B=G.value,B==="permissions_list"){if(globalThis.permissions_modal)globalThis.permissions_modal.remove();globalThis.permissions_modal=document.createElement("dialog"),globalThis.permissions_modal.id="permissions-modal",globalThis.permissions_modal.innerHTML=`<div style="width: 25rem; max-width: 100%;">
                    <h2 class="no-margin full text-center">Permissions</h2>
        
                    <hr />
                    <div class="flex flex-column g-4">
                        <button onclick="window.add_user_permissions()" class="round full border justify-start">
                            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-user-plus"><path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><line x1="19" x2="19" y1="8" y2="14"/><line x1="22" x2="16" y1="11" y2="11"/></svg>
                            Add User
                        </button>

                        <div id="permissions-modal-actions" class="flex flex-column g-4"></div>
                    </div>
                    <hr />
        
                    <div class="full flex justify-right">
                        <a class="button round red" href="javascript:document.getElementById('permissions-modal').close();">
                            Close
                        </a>
                    </div>
                </div>`,document.body.appendChild(globalThis.permissions_modal),globalThis.render_permissions_fields=()=>{document.getElementById("permissions-modal-actions").innerHTML="";for(let z of Object.entries(q.permissions_list))P(document.getElementById("permissions-modal-actions"),z[0],z[1])},globalThis.update_permissions_key=(z,J)=>{const N=J.target.options[J.target.selectedIndex];q.permissions_list[z]=N.value},globalThis.add_user_permissions=()=>{const z=prompt("Enter user name:");if(!z)return;q.permissions_list[z]="Normal",globalThis.render_permissions_fields()},globalThis.render_permissions_fields(),globalThis.remove_permission=(z)=>{delete q.permissions_list[z],globalThis.render_permissions_fields()},E=`<button class="theme:primary round" onclick="document.getElementById('permissions-modal').showModal();">Edit Permissions</button>`}let H=q[B];if(typeof H==="string"||H===null){const z=B==="about"||B==="page_template"?"textarea":"input";E=`<${z} 
                    type="text" 
                    name="${B}" 
                    placeholder="${B}" 
                    value="${z==="input"?H||"":""}" 
                    required 
                    oninput="window.paste_settings_field_input(event);" 
                    class="round mobile:max"
                    style="width: 60%;"
                ${z==="textarea"?`>${H||""}</textarea>`:"/>"}`,window.paste_settings_field_input=(J)=>{q[B]=J.target.value}}}F=K(q,B),L(h,F,E)};let F=K(q,B);L(h,F,E),I.addEventListener("submit",async(D)=>{D.preventDefault();const G=prompt("Please enter this paste's edit password:");if(!G)return;const z=await(await fetch("/api/metadata",{method:"POST",body:JSON.stringify({custom_url:A,edit_password:G,metadata:q}),headers:{"Content-Type":"application/json"}})).json();if(z.success===!1)return alert(z.message);else window.location.reload()}),M.addEventListener("click",()=>{const D=prompt("Enter field name:");if(!D)return;q[D]="unknown",F=K(q,B),L(h,F,E)})}var K=function(q,A){let h="";for(let C of Object.entries(q))h+=`<option value="${C[0]}" ${A===C[0]?"selected":""}>${C[0]}</option>\n`;return h},L=function(q,A,h){return q.innerHTML="",q.innerHTML+=`<select class="round mobile:max" onchange="window.change_current_property(event);" style="width: 38%;">
        <option value="">Select a field to edit</option>
        ${A}
    </select>${h}`,""},P=function(q,A,h){q.innerHTML+=`<div class="full flex justify-space-between align-center mobile:flex-column mobile:align-start g-4">
        <b style="min-width: max-content; max-width: 100%;">${A}</b>

        <div class="flex g-4" style="justify-content: flex-end;">
            <select class="round mobile:max" onchange="window.update_permissions_key('${A}', event);" style="width: 50%;">
                <option value="Normal" ${h==="Normal"?"selected":""}>Normal</option>
                <option value="EditTextPasswordless" ${h==="EditTextPasswordless"?"selected":""}>EditTextPasswordless</option>
                <option value="Passwordless" ${h==="Passwordless"?"selected":""}>Passwordless</option>
                <option value="Blocked" ${h==="Blocked"?"selected":""}>Blocked</option>
            </select>

            <button class="round red" title="Remove" onclick="window.remove_permission('${A}');" style="height: 40px !important;">
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-x"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
            </button>
        </div>
    </div>`};function Q(q){R(q,[["bundles:user.ForceClientTheme","Force Client Theme",!1],["bundles:user.DisableImages","Disable Images",!1],["bundles:user.DisableAnimations","Disable Animations",!1],["bundles:user.DisableCustomPasteCSS","Disable Paste CSS",!1]])}var R=function(q,A){for(let h of A){if(!window.localStorage.getItem(h[0]))window.localStorage.setItem(h[0],`${h[2]}`);q.innerHTML+=`<div class="full flex mobile:flex-column g-4 justify-space-between">
            <b 
                class="flex align-center round mobile:max"
                style="width: 60%;"
            >
                ${h[1]}
            </b>

            <select class="round mobile:max" onchange="window.update_user_setting('${h[0]}', event);" style="width: 38%;">
                <option value="on" ${window.localStorage.getItem(h[0])==="true"?"selected":""}>on</option>
                <option value="off" ${window.localStorage.getItem(h[0])==="false"?"selected":""}>off</option>
            </select>
        </div>`}window.update_user_setting=(h,C)=>{const I=C.target.options[C.target.selectedIndex];if(!I)return;window.localStorage.setItem(h,I.value==="on"?"true":"false")}},S={paste_settings:O,user_settings:Q};export{Q as user_settings,O as paste_settings,S as default};
