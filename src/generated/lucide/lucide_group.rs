use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 7V5c0-1.1.9-2 2-2h2" ></ path > < path d = "M17 3h2c1.1 0 2 .9 2 2v2" ></ path > < path d = "M21 17v2c0 1.1-.9 2-2 2h-2" ></ path > < path d = "M7 21H5c-1.1 0-2-.9-2-2v-2" ></ path > < rect height = "5" width = "7" x = "7" rx = "1" y = "7" ></ rect > < rect height = "5" width = "7" x = "10" y = "12" rx = "1" ></ rect > < / > } } pub const LucideGroup : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;