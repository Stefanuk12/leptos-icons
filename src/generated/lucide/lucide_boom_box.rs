use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 9V5a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v4" ></ path > < path d = "M8 8v1" ></ path > < path d = "M12 8v1" ></ path > < path d = "M16 8v1" ></ path > < rect y = "9" height = "12" width = "20" rx = "2" x = "2" ></ rect > < circle cx = "8" r = "2" cy = "15" ></ circle > < circle r = "2" cy = "15" cx = "16" ></ circle > < / > } } pub const LUCIDE_BOOM_BOX : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;