use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" width = "16" height = "16" x = "4" y = "3" ></ rect > < path d = "M4 11h16" ></ path > < path d = "M12 3v8" ></ path > < path d = "m8 19-2 3" ></ path > < path d = "m18 22-2-3" ></ path > < path d = "M8 15h.01" ></ path > < path d = "M16 15h.01" ></ path > < / > } } pub const LUCIDE_TRAM_FRONT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;