use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" width = "18" height = "18" x = "3" y = "3" ></ rect > < path d = "m16 12-4-4-4 4" ></ path > < path d = "M12 16V8" ></ path > < / > } } pub const LUCIDE_SQUARE_ARROW_UP : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;