use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8.12 8.12 12 12" ></ path > < path d = "M20 4 8.12 15.88" ></ path > < circle cy = "18" r = "3" cx = "6" ></ circle > < path d = "M14.8 14.8 20 20" ></ path > < / > } } pub const LucideScissors : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;