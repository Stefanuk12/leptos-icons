use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "22" y1 = "2" x2 = "22" x1 = "2" ></ line > < path d = "M7 7H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h16" ></ path > < path d = "M9.5 4h5L17 7h3a2 2 0 0 1 2 2v7.5" ></ path > < path d = "M14.121 15.121A3 3 0 1 1 9.88 10.88" ></ path > < / > } } pub const LUCIDE_CAMERA_OFF : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24")] } ;