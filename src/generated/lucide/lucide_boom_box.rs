use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 9V5a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v4" ></ path > < path d = "M8 8v1" ></ path > < path d = "M12 8v1" ></ path > < path d = "M16 8v1" ></ path > < rect height = "12" y = "9" rx = "2" x = "2" width = "20" ></ rect > < circle cy = "15" cx = "8" r = "2" ></ circle > < circle cx = "16" cy = "15" r = "2" ></ circle > < / > } } pub const LUCIDE_BOOM_BOX : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;