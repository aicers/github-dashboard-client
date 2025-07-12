use yew::prelude::*;

#[function_component(Profile)]
pub fn profile() -> Html {
    html! {
        <div>
            <h1>{"Profile"}</h1>
            <p>{"Manage your profile information and GitHub connections."}</p>
            <div class="profile-content">
                <div class="profile-section">
                    <h3>{"User Information"}</h3>
                    <div class="form-group">
                        <label>{"Display Name:"}</label>
                        <input type="text" placeholder="Enter your display name" />
                    </div>
                    <div class="form-group">
                        <label>{"Bio:"}</label>
                        <textarea placeholder="Tell us about yourself" rows="3"></textarea>
                    </div>
                </div>

                <div class="profile-section">
                    <h3>{"GitHub Connection"}</h3>
                    <p>{"Status: Connected"}</p>
                    <button class="btn-secondary">{"Reconnect GitHub"}</button>
                </div>

                <div class="profile-section">
                    <h3>{"Preferences"}</h3>
                    <div class="form-group">
                        <label>
                            <input type="checkbox" />
                            {" Show private repositories"}
                        </label>
                    </div>
                    <div class="form-group">
                        <label>
                            <input type="checkbox" />
                            {" Auto-refresh dashboard"}
                        </label>
                    </div>
                </div>

                <div class="profile-actions">
                    <button class="btn-primary">{"Save Changes"}</button>
                    <button class="btn-secondary">{"Reset"}</button>
                </div>
            </div>
        </div>
    }
}
