use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" x = "3" y = "3" width = "18" height = "18" ></ rect > < path d = "M21 9H3" ></ path > < path d = "M21 15H3" ></ path > < / > } } pub const LUCIDE_ROWS_3 : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;