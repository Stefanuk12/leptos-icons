use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 21a8 8 0 0 1 13.292-6" ></ path > < circle cy = "8" r = "5" cx = "10" ></ circle > < path d = "M19 16v6" ></ path > < path d = "M22 19h-6" ></ path > < / > } } pub const LucideUserRoundPlus : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;