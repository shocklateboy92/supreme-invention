import * as React from "react";
import Board from "./Board";
import * as Contracts from "./contracts/backend";

interface IBoardState {
    error?: string;
    data?: Contracts.Backend;
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

        if (this.state.data) {
            return <Board data={this.state.data} />;
        }

        return <p>Please Stand By...</p>;
    }

    public async componentDidMount() {
        const response = await fetch(`http://hyena-mkii.home.lasath.org:8080/`);
        if (response.status !== 200) {
            this.setState({
                error: `Backend server returned ${response.statusText}`
            });
        }

        const data = await response.text();
        this.setState({ data: Contracts.Convert.toBackend(data) });
    }
}
