use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6.3 2.84A1.5 1.5 0 0 0 4 4.11v11.78a1.5 1.5 0 0 0 2.3 1.27l9.344-5.891a1.5 1.5 0 0 0 0-2.538L6.3 2.841Z" ></ path > < / > } } pub const HeroiconsMiniSolidPlay : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("fill" , "currentColor") , ("viewBox" , "0 0 20 20") , ("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;