use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" x = "2" height = "16" width = "20" y = "4" ></ rect > < path d = "M6 8h.01" ></ path > < path d = "M10 8h.01" ></ path > < path d = "M14 8h.01" ></ path > < / > } } pub const LUCIDE_APP_WINDOW_MAC : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;