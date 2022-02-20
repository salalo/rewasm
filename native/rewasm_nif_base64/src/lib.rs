use base64;
use std::str;

#[rustler::nif]
pub fn decode() -> String {
    let encoded = String::from("Q2FuIHlvdSBwbGVhc2UKU2l0IHRoZSBmdWNrIGRvd24KUHJvdGVzdGluZyBpbiB5b3VyClBhcGVyIGNyb3duCllvdSBsb3ZlIHRvIGZlZWwgb2ZmZW5kZWQKRmlnaHRpbmcgZnJvbSBjb21wdXRlciB0cmVuY2hlcwoKW0Nob3J1c10KWW91IGdvdCBhIHNlbWktYXV0b21hdGljIG1vdXRoCkdvIGVhc3kgbm93CkFuZCB5b3UncmUgc28gbG91ZApBbmQgeW91J3JlIHNvIGxvdWQKCltWZXJzZSAyXQpFdmVyeW9uZSdzIHllbGxpbmcKSXQncyBjcmFja2luZyB0aGUgY2VpbGluZwpQYWludCBwZWVsaW5nCkxpa2UgYmFuYW5hIGZsb29yIHNwbGl0dGluZwpXb29kIHNwbGludGVyaW5nClRoZSBub2lzZSBpcyBkZWFmZW5pbmcKUmluZyB0aGUgYWxhcm0gYW5vdGhlciBzb3VuZApJcyBjb3B1bGF0aW5nClNwcmVhZCBlYWdsZSBvbiBteSBlYXJkcnVtcwpDaG9raW5nIG9uIFR3aXR0ZXIgY3VtCkkgbWlzcyBzdW1tZXIgZG9sZHJ1bXMKSSBtaXNzIHNvbmljIGJvcmVkb20KSSBNaXNzIENvbmdlbmlhbGl0eQpTdHJhbmdsZWQgaW5jZXNzYW50bHkKQnkgdGhlIHNwZWFrZXIgaHVtClNjcmVhbWluZyBtb25rcwpFYXJzIGFyZSBudW1iCkdvZCBjYW4ndCB3aGlzcGVyCldoZW4gdGhlIGJhc3MgaXMgdXAKW0Nob3J1c10KCltWZXJzZSAzXQpDYW4geW91IHBsZWFzZQpUdXJuIHlvdXJzZWxmIGRvd24KUml0YWxpbiBraWRzCkRvaW5nIGNvY2FpbmUgd2lwZSBvdXRzCk5vIG5lZWQgdG8gZmVlbCBvZmZlbmRlZApCb3JuIHRvIGEgdGltZQpXaGVuIHRoZSBxdWlldCBlbmRlZAoKW0Nob3J1c10K");
    let decoded = base64::decode(&encoded).unwrap();

    str::from_utf8(&decoded[..]).unwrap().to_string()
}

#[rustler::nif]
pub fn encode() -> String {
    let txt = b"Can you please
Sit the fuck down
Protesting in your
Paper crown
You love to feel offended
Fighting from computer trenches

[Chorus]
You got a semi-automatic mouth
Go easy now
And you're so loud
And you're so loud

[Verse 2]
Everyone's yelling
It's cracking the ceiling
Paint peeling
Like banana floor splitting
Wood splintering
The noise is deafening
Ring the alarm another sound
Is copulating
Spread eagle on my eardrums
Choking on Twitter cum
I miss summer doldrums
I miss sonic boredom
I Miss Congeniality
Strangled incessantly
By the speaker hum
Screaming monks
Ears are numb
God can't whisper
When the bass is up
[Chorus]

[Verse 3]
Can you please
Turn yourself down
Ritalin kids
Doing cocaine wipe outs
No need to feel offended
Born to a time
When the quiet ended

[Chorus]
";
    let encoded = base64::encode(txt);

    encoded
}

rustler::init!("Elixir.REWASM.NIF.Base64", [decode, encode]);
