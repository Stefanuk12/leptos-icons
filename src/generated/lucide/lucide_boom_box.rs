use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 9V5a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v4" ></ path > < path d = "M8 8v1" ></ path > < path d = "M12 8v1" ></ path > < path d = "M16 8v1" ></ path > < rect width = "20" x = "2" height = "12" y = "9" rx = "2" ></ rect > < circle cy = "15" r = "2" cx = "8" ></ circle > < circle r = "2" cy = "15" cx = "16" ></ circle > < / > } } pub const LUCIDE_BOOM_BOX : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;