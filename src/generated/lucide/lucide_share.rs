use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8" ></ path > < polyline points = "16 6 12 2 8 6" ></ polyline > < line x1 = "12" y1 = "2" x2 = "12" y2 = "15" ></ line > < / > } } pub const LUCIDE_SHARE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;