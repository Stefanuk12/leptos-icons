use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" ></ path > < / > } } pub const LUCIDE_MOON : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;