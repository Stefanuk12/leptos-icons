use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2.7 10.3a2.41 2.41 0 0 0 0 3.41l7.59 7.59a2.41 2.41 0 0 0 3.41 0l7.59-7.59a2.41 2.41 0 0 0 0-3.41l-7.59-7.59a2.41 2.41 0 0 0-3.41 0Z" ></ path > < / > } } pub const LUCIDE_DIAMOND : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none")] } ;