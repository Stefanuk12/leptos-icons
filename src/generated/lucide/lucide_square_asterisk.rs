use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" x = "3" y = "3" rx = "2" width = "18" ></ rect > < path d = "M12 8v8" ></ path > < path d = "m8.5 14 7-4" ></ path > < path d = "m8.5 10 7 4" ></ path > < / > } } pub const LUCIDE_SQUARE_ASTERISK : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none")] } ;