use yew::prelude::*;

#[function_component(Settings)]
pub fn settings() -> Html {
    html! {
        <div>
            <h1>{"Settings"}</h1>
            <p>{"Configure your dashboard preferences here."}</p>
            <div class="settings-content">
                <div class="setting-group">
                    <h3>{"Notifications"}</h3>
                    <label>
                        <input type="checkbox" />
                        {" Enable email notifications"}
                    </label>
                    <br/>
                    <label>
                        <input type="checkbox" />
                        {" Enable browser notifications"}
                    </label>
                </div>

                <div class="setting-group">
                    <h3>{"Display"}</h3>
                    <label>
                        {"Theme: "}
                        <select>
                            <option value="light">{"Light"}</option>
                            <option value="dark">{"Dark"}</option>
                            <option value="auto">{"Auto"}</option>
                        </select>
                    </label>
                    <br/>
                    <label>
                        {"Items per page: "}
                        <input type="number" value="20" min="10" max="100" />
                    </label>
                </div>

                <div class="setting-group">
                    <h3>{"GitHub Integration"}</h3>
                    <label>
                        {"Default repository view: "}
                        <select>
                            <option value="all">{"All repositories"}</option>
                            <option value="owned">{"Owned repositories"}</option>
                            <option value="collaborator">{"Collaborator repositories"}</option>
                        </select>
                    </label>
                </div>
            </div>
        </div>
    }
}
