use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 6h3" ></ path > < path d = "M17 6h.01" ></ path > < rect y = "2" rx = "2" width = "18" x = "3" height = "20" ></ rect > < circle cy = "13" cx = "12" r = "5" ></ circle > < path d = "M12 18a2.5 2.5 0 0 0 0-5 2.5 2.5 0 0 1 0-5" ></ path > < / > } } pub const LucideWashingMachine : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24")] } ;