use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 18h1.4c1.3 0 2.5-.6 3.3-1.7l6.1-8.6c.7-1.1 2-1.7 3.3-1.7H22" ></ path > < path d = "m18 2 4 4-4 4" ></ path > < path d = "M2 6h1.9c1.5 0 2.9.9 3.6 2.2" ></ path > < path d = "M22 18h-5.9c-1.3 0-2.6-.7-3.3-1.8l-.5-.8" ></ path > < path d = "m18 14 4 4-4 4" ></ path > < / > } } pub const LUCIDE_SHUFFLE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;