use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3Z" ></ path > < path d = "M19 10v2a7 7 0 0 1-14 0v-2" ></ path > < line y2 = "22" x2 = "12" x1 = "12" y1 = "19" ></ line > < / > } } pub const LUCIDE_MIC : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24")] } ;