use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m2 8 2 2-2 2 2 2-2 2" ></ path > < path d = "m22 8-2 2 2 2-2 2 2 2" ></ path > < path d = "M8 8v10c0 .55.45 1 1 1h6c.55 0 1-.45 1-1v-2" ></ path > < path d = "M16 10.34V6c0-.55-.45-1-1-1h-4.34" ></ path > < line y2 = "22" y1 = "2" x1 = "2" x2 = "22" ></ line > < / > } } pub const LucideVibrateOff : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;