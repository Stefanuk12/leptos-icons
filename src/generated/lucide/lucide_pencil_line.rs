use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 20h9" ></ path > < path d = "M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z" ></ path > < path d = "m15 5 3 3" ></ path > < / > } } pub const LUCIDE_PENCIL_LINE : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;