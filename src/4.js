function containsSameTwoAdjacentDigits(str, i = 0) {
  if (i + 1 < str.length) {
    if (str[i] === str[i + 1]) {
      const m = str.substring(i).match(/(.)\1{2,}/);
      if (m && m[1] === str[i]) {
        return containsSameTwoAdjacentDigits(str, i + m[0].length);
      }
      return true;
    }
    return containsSameTwoAdjacentDigits(str, i + 1);
  }
  return false;
}

function isNotDescending(str, i = 0) {
  if (i + 1 < str.length) {
    if (Number(str[i]) > Number(str[i + 1])) return false;
    return isNotDescending(str, i + 1);
  }
  return true;
}


(function main() {
  const min = 265275;
  const max = 781584;

  let pass = min;
  const passwords = [];

  while (pass < max) {
    if (
      containsSameTwoAdjacentDigits(pass.toString()) &&
      isNotDescending(pass.toString())
    ) {
      passwords.push(pass);
    }
    pass += 1;
  }

  console.log(passwords.length);
})();
