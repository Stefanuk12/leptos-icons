use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3Z" ></ path > < path d = "M19 10v2a7 7 0 0 1-14 0v-2" ></ path > < line x2 = "12" y1 = "19" x1 = "12" y2 = "22" ></ line > < / > } } pub const LUCIDE_MIC : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;