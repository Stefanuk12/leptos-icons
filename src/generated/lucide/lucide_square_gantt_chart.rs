use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" y = "3" height = "18" x = "3" rx = "2" ></ rect > < path d = "M9 8h7" ></ path > < path d = "M8 12h6" ></ path > < path d = "M11 16h5" ></ path > < / > } } pub const LUCIDE_SQUARE_GANTT_CHART : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;