use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z" ></ path > < path d = "M9 12h6" ></ path > < path d = "M12 9v6" ></ path > < / > } } pub const LUCIDE_SHIELD_PLUS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;