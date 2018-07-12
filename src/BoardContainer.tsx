import * as React from "react";

interface IBoardState {
    error?: string;
    data?: any;
}

export default class BoardContainer extends React.Component<{}, IBoardState> {
    constructor(props: {}) {
        super(props);

        this.state = {};
    }

    public render() {
        if (this.state.error) {
            return (
                <div className="error">
                    <h1>Oh noes!</h1>
                    <p>{this.state.error}</p>
                </div>
            );
        }

        return null;
    }

    public async componentDidMount() {
        try {
            const response = await fetch(
                `http://hyena-mkii.home.lasath.org:8080/`
            );
            if (response.status !== 200) {
                this.setState({
                    error: `Backend server returned ${response.statusText}`
                });
            }

            this.setState({
                data: await response.json()
            });
        } catch (exception) {
            this.setState({
                error: `Unable to comminicate with backend server: ${
                    exception.message
                }`
            });
        }
    }
}
