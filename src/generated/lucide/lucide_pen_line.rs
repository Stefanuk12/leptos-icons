use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 20h9" ></ path > < path d = "M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z" ></ path > < / > } } pub const LUCIDE_PEN_LINE : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;