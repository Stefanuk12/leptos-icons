use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 16h-3a2 2 0 0 0-2 2v3" ></ path > < path d = "M21 21v.01" ></ path > < path d = "M12 7v3a2 2 0 0 1-2 2H7" ></ path > < path d = "M3 12h.01" ></ path > < path d = "M12 3h.01" ></ path > < path d = "M12 16v.01" ></ path > < path d = "M16 12h1" ></ path > < path d = "M21 12v.01" ></ path > < path d = "M12 21v-1" ></ path > < / > } } pub const LucideQrCode : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;