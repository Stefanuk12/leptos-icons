use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 12v4a1 1 0 0 1-1 1h-4" ></ path > < path d = "M17 3h2a2 2 0 0 1 2 2v2" ></ path > < path d = "M17 8V7" ></ path > < path d = "M21 17v2a2 2 0 0 1-2 2h-2" ></ path > < path d = "M3 7V5a2 2 0 0 1 2-2h2" ></ path > < path d = "M7 17h.01" ></ path > < path d = "M7 21H5a2 2 0 0 1-2-2v-2" ></ path > < rect height = "5" y = "7" rx = "1" x = "7" width = "5" ></ rect > < / > } } pub const LUCIDE_SCAN_QR_CODE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;