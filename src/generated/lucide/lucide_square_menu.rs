use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" width = "18" height = "18" rx = "2" x = "3" ></ rect > < path d = "M7 8h10" ></ path > < path d = "M7 12h10" ></ path > < path d = "M7 16h10" ></ path > < / > } } pub const LUCIDE_SQUARE_MENU : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;