use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 18V5l12-2v13" ></ path > < path d = "m9 9 12-2" ></ path > < circle cy = "18" r = "3" cx = "6" ></ circle > < circle cy = "16" r = "3" cx = "18" ></ circle > < / > } } pub const LUCIDE_MUSIC_4 : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;