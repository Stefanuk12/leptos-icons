use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "4" cy = "12" cx = "12" ></ circle > < path d = "M12 4h.01" ></ path > < path d = "M20 12h.01" ></ path > < path d = "M12 20h.01" ></ path > < path d = "M4 12h.01" ></ path > < path d = "M17.657 6.343h.01" ></ path > < path d = "M17.657 17.657h.01" ></ path > < path d = "M6.343 17.657h.01" ></ path > < path d = "M6.343 6.343h.01" ></ path > < / > } } pub const LucideSunDim : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;