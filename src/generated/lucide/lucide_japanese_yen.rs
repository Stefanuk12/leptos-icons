use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 9.5V21m0-11.5L6 3m6 6.5L18 3" ></ path > < path d = "M6 15h12" ></ path > < path d = "M6 11h12" ></ path > < / > } } pub const LUCIDE_JAPANESE_YEN : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor")] } ;