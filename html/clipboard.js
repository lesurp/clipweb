var api_url = "/api";

function post(clipboard, callback) {
	var xhr = new XMLHttpRequest();
	xhr.onreadystatechange = function () {
		if (this.readyState != 4)  { return; }

		if (this.status == 200) {
			var data = JSON.parse(this.responseText);
			callback(data);
		}
	};
	xhr.open("POST", api_url, true);
	xhr.setRequestHeader('Content-Type', 'application/json');
	xhr.send(JSON.stringify(clipboard));
}

function get(callback) {
	var xhr = new XMLHttpRequest();
	xhr.onreadystatechange = function () {
		if (this.readyState != 4)  { return; }

		if (this.status == 200) {
			var data = JSON.parse(this.responseText);
			callback(data);
		}
	};
	xhr.open('GET', api_url, true);
	xhr.send();
}

function set_text(text) {
	document.getElementById("clipboard").value = text;
}

function get_text() {
	return document.getElementById("clipboard").value;
}

function get_clipboard() {
	get((get_response) => {
		set_text(get_response.content);
	});
}

function post_clipboard() {
	post(get_text(), (post_response) => {
		if(post_response == "Ok") return;
		if(typeof post_response.TooLong !== "undefined") {
			get_clipboard();
			set_error_status("input text was too long; " + post_response.TooLong + " characters allowed");
		}
	});
}

function set_error_status(message) {
	document.getElementById("status_bar").innerHTML = "Error: " + message;
	unfade(document.getElementById("status_bar"));
	setTimeout(() => {
		fade(document.getElementById("status_bar"));
	}, 3000);
}

get_clipboard();
document.getElementById("clipboard").oninput = post_clipboard;
