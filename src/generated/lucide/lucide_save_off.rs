use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M13 13H8a1 1 0 0 0-1 1v7" ></ path > < path d = "M14 8h1" ></ path > < path d = "M17 21v-4" ></ path > < path d = "m2 2 20 20" ></ path > < path d = "M20.41 20.41A2 2 0 0 1 19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 .59-1.41" ></ path > < path d = "M29.5 11.5s5 5 4 5" ></ path > < path d = "M9 3h6.2a2 2 0 0 1 1.4.6l3.8 3.8a2 2 0 0 1 .6 1.4V15" ></ path > < / > } } pub const LUCIDE_SAVE_OFF : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;