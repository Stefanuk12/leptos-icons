use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" x = "4" height = "16" y = "3" rx = "2" ></ rect > < path d = "M4 11h16" ></ path > < path d = "M12 3v8" ></ path > < path d = "m8 19-2 3" ></ path > < path d = "m18 22-2-3" ></ path > < path d = "M8 15h0" ></ path > < path d = "M16 15h0" ></ path > < / > } } pub const LucideTramFront : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24")] } ;