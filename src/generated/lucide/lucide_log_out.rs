use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" ></ path > < polyline points = "16 17 21 12 16 7" ></ polyline > < line x2 = "9" x1 = "21" y2 = "12" y1 = "12" ></ line > < / > } } pub const LUCIDE_LOG_OUT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;