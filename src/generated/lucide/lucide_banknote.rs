use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" width = "20" x = "2" y = "6" height = "12" ></ rect > < circle r = "2" cx = "12" cy = "12" ></ circle > < path d = "M6 12h.01M18 12h.01" ></ path > < / > } } pub const LUCIDE_BANKNOTE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;