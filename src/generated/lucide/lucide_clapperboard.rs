use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20.2 6 3 11l-.9-2.4c-.3-1.1.3-2.2 1.3-2.5l13.5-4c1.1-.3 2.2.3 2.5 1.3Z" ></ path > < path d = "m6.2 5.3 3.1 3.9" ></ path > < path d = "m12.4 3.4 3.1 4" ></ path > < path d = "M3 11h18v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2Z" ></ path > < / > } } pub const LUCIDE_CLAPPERBOARD : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;