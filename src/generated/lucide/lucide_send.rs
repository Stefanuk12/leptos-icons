use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m22 2-7 20-4-9-9-4Z" ></ path > < path d = "M22 2 11 13" ></ path > < / > } } pub const LUCIDE_SEND : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;