use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "16" r = "1" cx = "12" ></ circle > < rect x = "3" width = "18" y = "10" height = "12" rx = "2" ></ rect > < path d = "M7 10V7a5 5 0 0 1 9.33-2.5" ></ path > < / > } } pub const LucideUnlockKeyhole : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24")] } ;