use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 22h8" ></ path > < path d = "M12 11v11" ></ path > < path d = "m19 3-7 8-7-8Z" ></ path > < / > } } pub const LUCIDE_MARTINI : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;