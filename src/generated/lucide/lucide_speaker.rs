use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 6h.01" ></ path > < circle cy = "14" r = "4" cx = "12" ></ circle > < path d = "M12 14h.01" ></ path > < / > } } pub const LucideSpeaker : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round")] } ;