use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 9.5V21m0-11.5L6 3m6 6.5L18 3" ></ path > < path d = "M6 15h12" ></ path > < path d = "M6 11h12" ></ path > < / > } } pub const LUCIDE_JAPANESE_YEN : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;