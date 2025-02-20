from matplotlib import pyplot as plt


start_pop = 300000000 #how many people start healthy
start_infected = 1 # how many people start infected
days_to_model = 300# How many days should the simulation last
dt = 1 # simulation step size (in days) (smaller = more accurate)

infection_rate = (3.0/14.0) # people an infected person infects per day, assuming everyone they meet is susceptible
recovery_rate = (1.0/14.0) # probability an infected person recovers per day

t = 0  #initialization
susceptible = []
resistant = []
infected = []
time = []
sus =  start_pop
res =  0
inf =  start_infected

while t <= days_to_model:
     # First, record results so far, for plotting
    time.append(t)
    susceptible.append(sus)
    resistant.append(res)
    infected.append(inf)

    # The diffeq stuff happens here. 
    t += dt  #keep track of the current time, for plotting purposes
    proportion_susceptible = sus/(start_pop + start_infected)    # I factored this part out

    # calculate new values from old       
    inf_new =inf + dt*(infection_rate*inf*proportion_susceptible - recovery_rate*inf ) 
    print(str(t),str(inf),str(inf_new))
    sus_new = sus - dt*infection_rate*inf*proportion_susceptible
    res_new = res + dt*recovery_rate*inf

    #copy the new values over
    inf = inf_new
    sus = sus_new
    res = res_new
    
plt.plot(time,infected,time,susceptible,time,resistant)
plt.legend(["infected","susceptible",  "resistant"])
plt.show()

