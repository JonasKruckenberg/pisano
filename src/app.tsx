export function App() {
  return (
    <>
      <div class="circles">
        {[...Array(500).keys()].map((modulus) => (
          <label class="circle">
            {modulus + 1}
            <img
              src={`circleplot://localhost?modulus=${modulus + 1}`}
            />
          </label>
        ))}
      </div>
    </>
  );
}
