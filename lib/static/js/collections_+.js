onInput();

function onInput() {
    console.log('loaded');
    const input = document.getElementById('name');
    const button = document.getElementById('submit');
    const internalName = document.getElementById('collection_name');
    let disabled = true;

    internalName.innerHTML = toInternalName(input.value);

    input.addEventListener('keypress', e => {
        if (e.key === 'Enter') return e.preventDefault();

        const value = e.target.value;
        const invalid = 
            value.length === 0 && e.key === ' ' || 
            value[value.length - 1] === ' ' && e.key === ' ' ||
            !/[a-z ]/i.test(e.key);

        if (invalid) e.preventDefault();
    });

    input.addEventListener('keyup', e => {
        const value = e.target.value;
        if (value.trim().length > 0 && disabled) {
            disabled = false;
            button.disabled = false;
            button.classList.remove('disabled');
        } else if (value.trim().length === 0 && !disabled) {
            disabled = true;
            button.disabled = true;
            button.classList.add('disabled');
        }
        internalName.innerHTML = toInternalName(value);
    });
};

function toInternalName(name) {
    if (name.length === 0) return 'Stored as:';
    let value = name.trim().replace(/[^a-z]/gi, '_').toLowerCase();
    value = value.replace(/_+/g, '_');
    return `Stored as: ${value}`;
}