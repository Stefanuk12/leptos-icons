use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 9V5a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v4" ></ path > < path d = "M8 8v1" ></ path > < path d = "M12 8v1" ></ path > < path d = "M16 8v1" ></ path > < rect x = "2" height = "12" width = "20" rx = "2" y = "9" ></ rect > < circle cx = "8" cy = "15" r = "2" ></ circle > < circle cx = "16" cy = "15" r = "2" ></ circle > < / > } } pub const LucideBoomBox : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2")] } ;