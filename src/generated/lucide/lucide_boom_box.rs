use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 9V5a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v4" ></ path > < path d = "M8 8v1" ></ path > < path d = "M12 8v1" ></ path > < path d = "M16 8v1" ></ path > < rect height = "12" width = "20" x = "2" y = "9" rx = "2" ></ rect > < circle r = "2" cx = "8" cy = "15" ></ circle > < circle cy = "15" r = "2" cx = "16" ></ circle > < / > } } pub const LUCIDE_BOOM_BOX : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round")] } ;